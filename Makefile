raytracer:
	rm -f image.ppm && cargo run --quiet > image.ppm && open image.ppm