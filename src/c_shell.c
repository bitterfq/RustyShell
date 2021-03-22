#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/wait.h> 
#include<fcntl.h>







#define CMDLINE_MAX 512

/*
 *
  A struct that holds (in order):
  
  1. The initial command 
  2. The arguents of the command
  3. The ending null terminator


 */

struct parsedInput {


	// if equal to 1
	// than the command was cd
	int cdActive;

	// tracks total arguments
	int totalArgs;

	int argError;

	// flag to see if cd activated 
	int cd;

	char **c;

	// holds the file descriptor
	int fd;

};






// this is a function to change the stdout
// using DUP2

// I need to find a way to create a file,
// if the file is created already truncate it
// I will also need an int array to hold all
// of the file desvriptios


struct parsedInput changeStdOutput(struct parsedInput output, int position){
	// switch the stdout with the file descriptor 



	output.fd = open(output.c[position], O_RDWR 
	| O_TRUNC | O_CREAT, 0777);


	dup2(output.fd, 1);

	return output;
}

// parse the command and put it into a struct
struct parsedInput parser(char cmd[]){

	// struct to hold input
	struct parsedInput Input;


	// used a filler for sprintf
	char filler[2];

	
	// This is used to track different strings
	// for execvp
	int k = 0;
	int flag = 0;


	// allocate memory 
	Input.c = calloc(16, sizeof(char*));
	Input.c[k] = malloc(33);

	// This is a flag used to
	// see if the first command was parsed
	// 0: not parsed 
	// 1 parsed 


	int realArg = 0;

	
	for(int i = 0; cmd[i] != '\0'; i++){

		if (realArg == 1 && cmd[i] == ' ' && flag == 0){
			k++;
			// allocate memory for the next string
			Input.c[k] = malloc(33);

			flag = 1;
		}

		if (cmd[i] != ' '){
			sprintf(filler,"%c", cmd[i]);
			strcat(Input.c[k], filler);
			flag = 0;
			realArg = 1;
		}

	}


	if(strcmp(Input.c[0], ">") == 0 || strcmp(Input.c[0], "|") == 0){
		
		fprintf(stderr, "Error: missing command\n");
		Input.argError = 1;
		return Input;
	}


	// holds position of the output redirect
	int position;
	int count = 0;
	while(Input.c[count]){
		
		if(strcmp(Input.c[count], ">") == 0 && Input.c[count+1]){
			

			struct parsedInput temp;
			position = count + 1;
			temp = changeStdOutput(Input, position);
			Input.fd = temp.fd;
			Input.c[count] = '\0';
		}
		count++;


	}


	// check if cd is run correctly
	if(strcmp("cd", Input.c[0])== 0){
		Input.cdActive = 1;

		int dir = chdir(Input.c[1]);

		if(dir == -1){
			fprintf(stderr, "Error: cannot cd into directory\n");
			Input.cd = 0;
			return Input;	
		} 
		
		if(dir == 0){
			Input.cd = 1;
			return Input;
		}
	}

	// No need to go forward if their 
	// is only one command
	if(Input.c[1] == NULL){

		return Input;

	}



	return Input;
}	


//ref: fork_exec_wait, our implementation of system()
int sys(struct parsedInput Input){
        pid_t pid;
        int status;

	// holds value of execvp 
	int exVal;

        pid = fork();

        if (pid == 0) {

		if(Input.totalArgs == 17){
			
			fprintf(stderr, "Error: too many process arguments\n");

		}


                exVal = execvp(Input.c[0], Input.c);

		if(exVal == -1 && (strcmp(Input.c[0], "cd") != 0)){
			
			fprintf(stderr, "Error: command not found\n");

		}

                exit(1);
        }else if (pid > 0) {
                /* Parent */
                waitpid(pid, &status, 0);
		
		close(Input.fd);


		// child process always restores
		// the shell prompt
		dup2(STDIN_FILENO,1);



        } else {
                perror("fork");
                exit(1);
        }

        // return the actual exit status of execvp process.
        return WEXITSTATUS(status) ;
}


int main(void)
{
        char cmd[CMDLINE_MAX];


        while (1) {
                char *nl;
                int retval;

                /* Print prompt */
                printf("sshell$ ");
                fflush(stdout);

                /* Get command line */
                fgets(cmd, CMDLINE_MAX, stdin);



		// for no argument
		if(strlen(cmd) == 1){
			continue;

		}
		

                /* Print command line if stdin is not provided by terminal */
                if (!isatty(STDIN_FILENO)) {
                        printf("%s", cmd);
                        fflush(stdout);
                }

                /* Remove trailing newline from command line */
                nl = strchr(cmd, '\n');
                if (nl)

                        *nl = '\0';

                /* Builtin command */
                if (!strcmp(cmd, "exit")) {
                        fprintf(stderr, "Bye...\n");

                	fprintf(stderr, "+ completed 'exit' [0]\n");
                        break;
                }

		// if input is pwd
		if(strcmp("pwd", cmd) == 0){
		
			
			printf("%s\n", getcwd(cmd, sizeof(cmd)));


                	fprintf(stderr, "+ completed 'pwd' [0]\n");
			continue;

		}
		struct parsedInput Input = parser(cmd);

		if(Input.argError == 1){
			continue;

		}
		
		// display if cd was successful 
		if(Input.cd == 1){
			
                	fprintf(stderr, "+ completed '%s' [0]\n",cmd);

			continue;


		}




                /* Regular command */
                retval = sys(Input);
                fprintf(stderr, "+ completed '%s' [%i]\n",
                        cmd, retval);
                //fprintf(stdout, "Return status value for '%s': %d\n",cmd, retval);

        }

        return EXIT_SUCCESS;
}
