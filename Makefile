serve:
	trunk serve --open

deployDist:
	rm -rf docs
	trunk build --release -d ./docs --public-url resume
	touch .nojekyll && cp .nojekyll ./docs
	git add .
	git commit -m 'deployment'
	git push
