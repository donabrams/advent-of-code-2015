#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <regex.h>

#define CMD_TURN_ON "turn on"
#define CMD_TURN_OFF "turn off"
#define CMD_TOGGLE "toggle"

char *copyGroup(regmatch_t group, char * line);

int **getLights();
void freeLights(int **a);

int toIntAndFree(char * s);

void turnOn(int **lights, int x1, int y1, int x2, int y2);
void turnOff(int **lights, int x1, int y1, int x2, int y2);
void toggle(int **lights, int x1, int y1, int x2, int y2);

int brightness(int **lights);
