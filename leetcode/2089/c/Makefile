CXX := gcc
CXXFLAGS := -g -Wall -Wextra -std=c11

main: main.o
	$(CXX) -o main main.o

main.o: main.c
	$(CXX) $(CXXFLAGS) -c main.c

run: main
	./main

clean:
	rm -f main main.o
	@echo "Cleaned up build files."

.PHONY: all clean run
