#ifndef SOCKET_H
#define SOCKET_H

#include "core.h"

#define SOCKET_PATH "/tmp/prex.socket"
#define EXE_LEN 128
#define ARG_LEN 128

typedef struct
{
  enum
  {
    CMD_DAEMON,
    CMD_EXEC,
    CMD_ARG
  } type;

  union
  {
    struct
    {
      bool off;
    } daemon;

    struct
    {
      char name[EXE_LEN];
      int argc;
    } exec;

    struct
    {
      char arg[ARG_LEN];
    } arg;
  } as;
} Command;

#define CMD_LEN (sizeof (Command))

#endif
