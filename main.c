#include <stdio.h>
#include <stdlib.h>

int main(int argc, char** argv) {
    if (argc != 3) {
        printf("Usage: bfc [input filename] [output filename]\n");
        return 1;
    }

    FILE *infile = fopen(argv[1], "r");
    FILE *outfile = fopen("intermediate.c", "w");

    if (infile == NULL || outfile == NULL) {
        perror("Error opening file");
        return 1;
    }

    fputs("#include <stdio.h>\n\n"
          "int main(void)\n"
          "{\n"
          "    unsigned char memory[30000] = {0};\n"
          "    unsigned int ptr = 0;\n",
          outfile);

    int c;
    while ((c = fgetc(infile)) != EOF) {
        switch (c) {
            case '<':
                fputs("ptr--;", outfile);
                break;
            case '>':
                fputs("ptr++;", outfile);
                break;
            case '+':
                fputs("memory[ptr]++;", outfile);
                break;
            case '-':
                fputs("memory[ptr]--;", outfile);
                break;
            case ',':
                fputs("memory[ptr] = getchar();", outfile);
                break;
            case '.':
                fputs("putchar(memory[ptr]);", outfile);
                break;
            case '[':
                fputs("while (memory[ptr]) {", outfile);
                break;
            case ']':
                fputs("}", outfile);
                break;
            default:
                break;
        }
    }

    fputs("\n    return 0;\n}\n", outfile);

    fclose(infile);
    fclose(outfile);

    char command[1024];
    snprintf(command, sizeof(command), "cc intermediate.c -o \"%s\"", argv[2]);

    if (system(command) != 0) {
        fprintf(stderr, "Error compiling outputted C code\n");
        return 1;
    }

	system("rm intermediate.c");

    return 0;
}
