#ifndef SOCKET_H
#define SOCKET_H

#include "core.h"

#define SOCKET_PATH "/tmp/prex.socket"

typedef struct
{
  enum
  {
    CMD_DAEMON,
    CMD_EXEC
  } type;

  union
  {
    struct
    {
      bool off;
    } daemon;
  } as;
} Command;

#define CMD_LEN (sizeof (Command))

#endif
