ALL = 3 4 9

.PHONY: all
all: $(ALL)

.PHONY: clean
clean:
	$(RM) $(ALL)

3: 3.cpp
	g++ -std=c++17 -Wall -Wextra -o $@ $<

4: 4.rs
	rustc -o $@ $<

9: 9.hs
	ghc -o $@ $<
