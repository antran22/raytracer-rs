complex:
	mkdir -p output && cargo run --quiet -- --mode fast --scene complex && open output/image.png
checkered_sphere:
	mkdir -p output && cargo run --quiet -- --mode fast --scene checkered-sphere && open output/image.png
earth:
	mkdir -p output && cargo run --quiet -- --mode fast --scene earth && open output/image.png
