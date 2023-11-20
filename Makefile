serve:
	trunk serve --open

deployDist:
	rm -rf docs
	trunk build --release -d ./docs --public-url resume
	git add .
	git commit -m 'deployment'
	git push
