main: main.o
	gcc -o main main.o

main.o: main.c
	gcc -c main.c

run: main
	./main

clean:
	rm -f main.o main
	@echo "Cleaned up object files and executable."

.PHONY: run clean
