build:
	cmake -B build -S . -G Ninja && cmake --build build

clean:
	cmake --build build --target clean && rm -rf build

.PHONY: build clean
