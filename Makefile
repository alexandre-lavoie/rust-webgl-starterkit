install:
	cd www && npm install

build:
	cd www && npm run build

start:
	cd www && npm start

clean:
	rm -rf pkg
	rm -rf dist