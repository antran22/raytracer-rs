raytracer:
	mkdir -p output && cargo run --quiet >| output/image.ppm && open output/image.ppm