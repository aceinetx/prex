#include "arena.h"
#include "exec.h"
#include "socket.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

Arena argv_arena;

int
main (void)
{
  argv_arena = arena_new ();
  {
    bool running;
    int server_fd, client_fd;
    struct sockaddr_un addr;
    Command data;

    /* create socket */
    server_fd = socket (AF_UNIX, SOCK_STREAM, 0);
    if (server_fd < 0)
      {
        perror ("socket");
        exit (EXIT_FAILURE);
      }

    /* remove any existing socket */
    unlink (SOCKET_PATH);

    /* set up the address structure */
    memset (&addr, 0, sizeof (struct sockaddr_un));
    addr.sun_family = AF_UNIX;
    strncpy (addr.sun_path, SOCKET_PATH, sizeof (addr.sun_path) - 1);

    /* bind the socket */
    if (bind (server_fd, (struct sockaddr *)&addr, sizeof (addr)) < 0)
      {
        perror ("bind");
        close (server_fd);
        exit (EXIT_FAILURE);
      }

    /* listen for connections */
    if (listen (server_fd, 5) < 0)
      {
        perror ("listen");
        close (server_fd);
        exit (EXIT_FAILURE);
      }

    puts ("Daemon is running...");

    running = true;
    while (running)
      {
        /* accept a connection */
        client_fd = accept (server_fd, NULL, NULL);
        if (client_fd < 0)
          {
            perror ("accept");
            close (server_fd);
            exit (EXIT_FAILURE);
          }

        /* read data from the client */
        read (client_fd, &data, CMD_LEN);
        puts ("Command recieved");

        if (data.type == CMD_DAEMON)
          {
            if (data.as.daemon.off)
              {
                puts ("Daemon is shutting down...");
                running = false;
              }
          }
        else if (data.type == CMD_EXEC)
          {
            int i;
            bool valid;
            size_t size
                = sizeof (char *)
                  * (data.as.exec.argc
                     + 2 /* one for a null terminator, one for exe name */);
            char **argv = malloc (size);
            memset (argv, 0, size);

            valid = true;

            argv[0] = strdup (data.as.exec.name);
            arena_append (&argv_arena, argv[0]);
            for (i = 0; i < data.as.exec.argc; i++)
              {
                Command arg;
                read (client_fd, &arg, CMD_LEN);

                if (arg.type != CMD_ARG)
                  {
                    printf ("Got an invalid command of ID %u (expected "
                            "CMD_ARG, or %u)\n",
                            (unsigned int)data.type, (unsigned int)CMD_ARG);
                    valid = false;
                    break;
                  }

                argv[i + 1] = strdup (arg.as.arg.arg);
                arena_append (&argv_arena, argv[i + 1]);
              }

            if (valid)
              {
                arena_append (&argv_arena, argv);

                prex_exec (argv);
              }
          }
        else
          {
            printf ("Got an invalid command of ID %u\n",
                    (unsigned int)data.type);
          }
      }

    /* clean up */
    close (client_fd);
    close (server_fd);
    unlink (SOCKET_PATH);
  }
  arena_free (&argv_arena);
  return 0;
}
