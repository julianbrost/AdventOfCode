ALL = 3 4

.PHONY: all
all: $(ALL)

.PHONY: clean
clean:
	$(RM) $(ALL)

3: 3.cpp
	g++ -std=c++17 -Wall -Wextra -o $@ $<

4: 4.rs
	rustc -o $@ $<
