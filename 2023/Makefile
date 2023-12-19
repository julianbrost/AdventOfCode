ALL = 3 4 11 15

.PHONY: all
all: $(ALL)

.PHONY: clean
clean:
	$(RM) $(ALL)

3: 3.cpp
	g++ -std=c++17 -Wall -Wextra -o $@ $<

4: 4.rs
	rustc -o $@ $<

11: 11.f90
	gfortran -o $@ $<

15: 15.cob
	cobc -O2 -x -o $@ $<
