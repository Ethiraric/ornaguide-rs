set dotenv-load := false

default:
  @just --list

new_jsons:
  (cd jsons && for x in item class specialization skill pet monster quest npc; do curl -X POST https://orna.guide/api/v1/${x} -d "{}" | jq . > ${x}.json; done)
  cp -r jsons/ jsons-`date "+%Y-%m-%d"` && BZIP2=-9 tar -cjvf jsons-`date "+%Y-%m-%d"`{.tar.bz2,} && mv jsons-`date "+%Y-%m-%d"`.tar.bz2 json_backups/ && rm -r jsons-`date "+%Y-%m-%d"`

backup_htmls:
  mv htmls htmls-`date "+%Y-%m-%d"`
  BZIP2=-9 tar -cjvf htmls-`date "+%Y-%m-%d"`{.tar.bz2,}
  mv htmls-`date "+%Y-%m-%d"`.tar.bz2 html_backups
  mv htmls-`date "+%Y-%m-%d"` htmls

