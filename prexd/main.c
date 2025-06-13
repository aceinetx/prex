#include "socket.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

int
main (void)
{
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

        switch (data.type)
          {
          case CMD_DAEMON:
            if (data.as.daemon.off)
              {
                running = false;
              }
            break;
          default:
            printf ("Got an invalid command of ID %u\n",
                    (unsigned int)data.type);
            break;
          }
      }

    /* clean up */
    close (client_fd);
    close (server_fd);
    unlink (SOCKET_PATH);
  }
  return 0;
}
