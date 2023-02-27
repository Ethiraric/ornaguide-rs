set dotenv-load := false

default:
  @just --list

new_jsons:
  (cd jsons && for x in item class specialization skill pet monster quest npc; do curl -X POST https://orna.guide/api/v1/${x} -d "{}" | jq . > ${x}.json; done)
  cp -r jsons/ jsons-`date "+%Y-%m-%d"` && BZIP2=-9 tar -cjvf jsons-`date "+%Y-%m-%d"`{.tar.bz2,} && mv jsons-`date "+%Y-%m-%d"`.tar.bz2 backups_json/ && rm -r jsons-`date "+%Y-%m-%d"`

backup_htmls:
  mv htmls htmls-`date "+%Y-%m-%d"`
  BZIP2=-9 tar -cjvf htmls-`date "+%Y-%m-%d"`{.tar.bz2,}
  mv htmls-`date "+%Y-%m-%d"`.tar.bz2 backups_html
  mv htmls-`date "+%Y-%m-%d"` htmls

backup_output:
  cargo run --release --bin ethi json refresh
  cp -r output output-`date "+%Y-%m-%d"`
  BZIP2=-9 tar -cjvf output-`date "+%Y-%m-%d"`{.tar.bz2,}
  mv output-`date "+%Y-%m-%d"`.tar.bz2 backups_output
  rm -r output-`date "+%Y-%m-%d"`

json_refresh:
  cargo run --release --bin ethi json refresh

backup_htmls_now:
  cp -r htmls htmls-`date "+%Y-%m-%dT%H-%M"`
  BZIP2=-9 tar -cjvf htmls-`date "+%Y-%m-%dT%H-%M"`{.tar.bz2,}
  mv htmls-`date "+%Y-%m-%dT%H-%M"`.tar.bz2 backups_html
  rm -r htmls-`date "+%Y-%m-%dT%H-%M"`

backup_output_now:
  cp -r output output-`date "+%Y-%m-%dT%H-%M"`
  BZIP2=-9 tar -cjvf output-`date "+%Y-%m-%dT%H-%M"`{.tar.bz2,}
  mv output-`date "+%Y-%m-%dT%H-%M"`.tar.bz2 backups_output
  rm -r output-`date "+%Y-%m-%dT%H-%M"`

merge:
  cargo run --release --bin ethi backups merge

quick_merge_now: backup_output_now merge

new_merge_now: json_refresh quick_merge_now

cron: new_jsons backup_htmls backup_output
