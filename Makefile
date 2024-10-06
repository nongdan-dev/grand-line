fmt:
	cargo fmt \
	&& dprint fmt;

update:
	cargo update --dry-run \
	&& cargo install-update -a \
	&& cargo upgrade --incompatible \
	&& dprint upgrade \
	&& dprint config update;

reinit:
	rm -rf .git \
	&& git init \
	&& git add -A \
	&& make -Bs fmt \
	&& make -Bs imagemin \
	&& git add -A \
	&& git commit -m "Initial commit" \
	&& git remote add origin git@github.com:nongdan-dev/grand-line.git \
	&& git push -uf origin master;

imagemin:
	export EXT="png|jpg|gif|ico" \
	&& make -Bs git-ls \
	| xargs -L1 bash -c 'imagemin $$0 --out-dir $$(dirname $$0)';
git-ls:
	bash -c 'comm -3 <(git ls-files) <(git ls-files -d)' \
	| egrep -h '\.($(EXT))$$';
