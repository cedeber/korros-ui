serve:
	cd out/ && python3 ../server.py

dev:
	watchexec -e rs,css -w styles -w crates -c "just debug"

debug:
	rm -rf out/assets/
	just crates/korros/debug
	esbuild --bundle --sourcemap --loader:.woff2=file --outfile=out/assets/styles.css styles/styles.css

release:
	rm -rf out/assets/
	just crates/korros/release
	esbuild --minify --outfile=out/assets/korros.js --allow-overwrite out/assets/korros.js
	esbuild --bundle --minify --loader:.woff2=file --outfile=out/assets/styles.css styles/styles.css