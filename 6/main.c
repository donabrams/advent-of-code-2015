#include "main.h"

int main(void)
{
	char * regexString = "(turn on|turn off|toggle) ([[:digit:]]+),([[:digit:]]+) through ([[:digit:]]+),([[:digit:]]+)";
	printf("%s\n", regexString);
	FILE * directions;
	char * line = NULL;
	size_t len = 0;
	ssize_t read;
	regex_t regexCompiled;

	// compile regex
	if (regcomp(&regexCompiled, regexString, REG_EXTENDED)) {
		perror("Error compiling regex");
		exit(EXIT_FAILURE);
    };
	regmatch_t matches[regexCompiled.re_nsub+1];
	printf("num subexpressions: %lu\n", regexCompiled.re_nsub);

    // read in the directions
	directions = fopen("directions.txt", "r");
	if (directions == NULL) {
		exit(EXIT_FAILURE);
	}

	int** lights = getLights();

	// for each line in the directions
	while ((read = getline(&line, &len, directions)) != -1) {
		char *command;
		int x1, y1, x2, y2;
		// extract out the params via the regex
		int status = regexec(&regexCompiled, line, regexCompiled.re_nsub+1, matches, 0);
		if (status == 0) {
			command = copyGroup(matches[1], line);
			x1 = toIntAndFree(copyGroup(matches[2], line));
			y1 = toIntAndFree(copyGroup(matches[3], line));
			x2 = toIntAndFree(copyGroup(matches[4], line));
			y2 = toIntAndFree(copyGroup(matches[5], line));
			printf("%s %d %d %d %d\n", command, x1, y1, x2, y2);
			if (strcmp(command, CMD_TURN_ON) == 0) {
				turnOn(lights, x1, y1, x2, y2);
			} else if (strcmp(command, CMD_TURN_OFF) == 0) {
				turnOff(lights, x1, y1, x2, y2);
			} else if (strcmp(command, CMD_TOGGLE) == 0) {
				toggle(lights, x1, y1, x2, y2);
			} else {
				perror("Unknown command");
			}
		}
	}

	printf("num lights lit: %d\n", numLitLights(lights));

	freeLights(lights);
  	regfree(&regexCompiled);
	fclose(directions);
	if (line)
		free(line);
	exit(EXIT_SUCCESS);
}


char *copyGroup(regmatch_t group, char * line)
{
	int length = group.rm_eo - group.rm_so;
	char *dest = malloc((length+1) * sizeof(char));
	strncpy(dest, line + group.rm_so, length);
	dest[length] = '\0';
	return dest;
}

int **getLights() {
	int** lights = malloc(1000*sizeof(int *));
	int i;
    for (i = 0; i < 1000; i++) {
        lights[i] = malloc(1000*sizeof(int));
        int j;
        for (j = 0; j < 1000; j++) {
        	lights[i][j] = LIGHT_OFF;
        }
    }
    return lights;
}

void freeLights(int **a) {
    for (int i = 0; i < 1000; i++) {
        free(a[i]);
    }
    free(a);
}

int toIntAndFree(char * s)
{
	int i = atoi(s);
	free(s);
	return i;
}


void turnOn(int **lights, int x1, int y1, int x2, int y2) {
	int x;
	for (x = x1; x <= x2; x++) {
		int y;
		for (y = y1; y <= y2; y++) {
			lights[x][y] = LIGHT_ON;
		}
	}
}

void turnOff(int **lights, int x1, int y1, int x2, int y2) {
	int x;
	for (x = x1; x <= x2; x++) {
		int y;
		for (y = y1; y <= y2; y++) {
			lights[x][y] = LIGHT_OFF;
		}
	}
}

void toggle(int **lights, int x1, int y1, int x2, int y2) {
	int x;
	for (x = x1; x <= x2; x++) {
		int y;
		for (y = y1; y <= y2; y++) {
			lights[x][y] = lights[x][y] == LIGHT_ON ? LIGHT_OFF : LIGHT_ON;
		}
	}
}

int numLitLights(int **lights) {
	int count = 0;
	int x;
	for (x = 0; x < 1000; x++) {
		int y;
		for (y = 0; y < 1000; y++) {
			if (lights[x][y] == LIGHT_ON) {
				count++;
			}
		}
	}
	return count;
}
