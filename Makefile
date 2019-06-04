deploy: book
	@echo "====> deploying to github"
	rm -rf /tmp/arsenal_book
	git worktree prune
	git worktree add /tmp/arsenal_book gh-pages
	rm -rf /tmp/arsenal_book/*
	cp -rp build/book/* /tmp/arsenal_book/
	cd /tmp/arsenal_book && \
		git add -A && \
		git commit -m "deployed on $(shell date) by ${USER}" && \
		git push origin gh-pages

book:
	mdbook build
