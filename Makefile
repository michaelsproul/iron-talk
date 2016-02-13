index.html: index.md
	pandoc $^ -t revealjs -s --variable theme=serif -o $@

open: index.html
	firefox $^

clean:
	rm -f index.html
