fmt:
	@cargo fmt \
	&& cargo fix --allow-dirty \
	&& doctoc --loglevel warn --github README.md \
	&& dprint fmt;

check:
	@make fmt \
	&& cargo clippy --fix --allow-dirty;

test:
	@make check \
	&& cargo test --features test_utils;

test_mysql:
	@make check \
	&& cargo test --no-default-features --features test_utils,default_without_db,mysql;

test_sqlite:
	@make check \
	&& bash ./tests/all.sh;

update:
	@cargo update --dry-run \
	&& cargo install-update -a \
	&& cargo upgrade --incompatible \
	&& dprint upgrade \
	&& dprint config update;

push:
	@git add -A \
	&& make fmt \
	&& make imagemin \
	&& git add -A \
	&& git commit -m "Update" \
	&& git push;

imagemin:
	@export EXT="png|jpg|gif|ico" \
	&& make git-ls \
	| xargs -L1 bash -c 'imagemin $$0 --out-dir $$(dirname $$0)';
git-ls:
	@bash -c 'comm -3 <(git ls-files) <(git ls-files -d)' \
	| egrep -h '\.($(EXT))$$';

# missing trailing comma
# ([^,\s.*])([\s\n]+\))
