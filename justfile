serve:
	cd out/ && python3 ../server.py

dev:
	watchexec -e rs,css -w styles -w crates -i crates/**/assets/* -c "just debug"

debug:
	rm -rf out/assets/
	rm -rf crates/korros/src/assets/
	# esbuild --bundle --outfile=crates/korros/src/assets/styles.css styles/styles.css
	mkdir crates/korros/src/assets/
	lightningcss --nesting --bundle --targets 'last 5 versions' styles/styles.css -o crates/korros/src/assets/styles.css
	just crates/playground/debug

release:
	rm -rf out/assets/
	rm -rf crates/korros/src/assets/
	# esbuild --bundle --minify --outfile=crates/korros/src/assets/styles.css styles/styles.css
	mkdir crates/korros/src/assets/
	lightningcss --nesting --bundle --minify --targets 'last 1 year' styles/styles.css -o crates/korros/src/assets/styles.css
	just crates/playground/release
	esbuild --minify --outfile=out/assets/korros.js --allow-overwrite out/assets/korros.js