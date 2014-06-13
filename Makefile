

.PHONY : all
all: met

.PHONY : met
met:
	rustc -g -L deps -o met src/main.rs

.PHONY : clean
clean:
	rm met
