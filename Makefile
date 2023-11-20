serve:
	trunk serve --open

buildLocal:
	trunk build --release

deployDist:
	trunk build --release --public-url resume
	touch .nojekyll && cp .nojekyll ./dist
	git add .
	git commit -m 'deployment'
	git push