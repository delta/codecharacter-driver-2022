EXE=run
CXX=g++
FLAGS=-std=c++20 -O2 -static -fdiagnostics-color=always

all: $(EXE)

player_code.o: player_code.h player_code.cpp
	$(CXX) -c -o player_code.o $(FLAGS) player_code.cpp

$(EXE).o: $(EXE).cpp player_code.h
	$(CXX) -c  -o $(EXE).o $(FLAGS) $(EXE).cpp

main.o: main.cpp player_code.h 
	$(CXX) $(FLAGS) -c -o main.o main.cpp

$(EXE): $(EXE).o player_code.o main.o
	g++ -fdiagnostics-color=always -static -o $(EXE) $(EXE).o player_code.o main.o

clean_objects:
	rm player_code.o $(EXE).o main.o

clean:
	rm $(EXE) $(EXE).o

.PHONY: all clean
