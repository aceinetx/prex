#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/errno.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

void
prex_exec (char *argv[])
{
  pid_t pid = fork ();

  if (pid < 0)
    {
      printf ("fork failed: %s", strerror (errno));
      return;
    }
  else if (pid == 0)
    {
      execvp (argv[0], argv);

      printf ("execvp failed: %s", strerror (errno));
      return;
    }
  else
    {
      printf ("%s process running in the background with PID: %d\n", argv[0],
              pid);
    }
}
