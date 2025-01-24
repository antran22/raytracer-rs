complex:
	mkdir -p output && cargo run --quiet -- --mode fast --scene complex && open output/image.png
quads:
	mkdir -p output && cargo run --quiet -- --mode fast --scene quads && open output/image.png
checkered_sphere:
	mkdir -p output && cargo run --quiet -- --mode fast --scene checkered-sphere && open output/image.png
checkered_sphere_slow:
	mkdir -p output && cargo run --quiet -- --mode slow --scene checkered-sphere && open output/image.png
earth:
	mkdir -p output && cargo run --quiet -- --mode fast --scene earth && open output/image.png
perlin:
	mkdir -p output && cargo run --quiet -- --mode fast --scene perlin && open output/image.png
