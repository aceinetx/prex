#include "socket.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

static char *
args_shift (int *argc, char ***argv)
{
  assert (*argc > 0 && "argc <= 0");
  --(*argc);
  return *(*argv)++;
}

int
main (int argc, char **argv)
{
  Command data;

  if (argc < 3)
    {
      puts ("usage: prex [command] (args)");
      puts ("            daemon    off");
      puts ("            exec      name [exec args...]");
      return 1;
    }

  args_shift (&argc, &argv);
  {
    char *cmd = args_shift (&argc, &argv);
    if (streq (cmd, "daemon"))
      {
        char *arg = args_shift (&argc, &argv);
        if (!streq (arg, "off"))
          {
            printf ("unacceptable argument for daemon: %s\n", arg);
            return 1;
          }
        data.type = CMD_DAEMON;
        data.as.daemon.off = true;
      }
    else if (streq (cmd, "exec"))
      {
      }
    else
      {
        printf ("unknown command: %s\n", cmd);
        return 1;
      }
  }

  {
    int sock_fd;
    struct sockaddr_un addr;

    /* create socket */
    sock_fd = socket (AF_UNIX, SOCK_STREAM, 0);
    if (sock_fd < 0)
      {
        perror ("socket");
        exit (EXIT_FAILURE);
      }

    /* set up the address structure */
    memset (&addr, 0, sizeof (struct sockaddr_un));
    addr.sun_family = AF_UNIX;
    strncpy (addr.sun_path, SOCKET_PATH, sizeof (addr.sun_path) - 1);

    /* connect to the server */
    if (connect (sock_fd, (struct sockaddr *)&addr, sizeof (addr)) < 0)
      {
        perror ("connect");
        close (sock_fd);
        exit (EXIT_FAILURE);
      }

    /* send data to the server */
    write (sock_fd, &data, CMD_LEN);

    /* clean up */
    close (sock_fd);
  }
  return 0;
}
