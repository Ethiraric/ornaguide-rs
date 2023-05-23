set dotenv-load := false

export NOW := `date "+%Y-%m-%d"`
export NOWDT := `date "+%Y-%m-%dT%H-%M"`

default:
  @just --list

new_jsons:
  mkdir -p data/current_entries/guide_api
  for x in item class specialization skill pet monster quest npc; do curl -X POST https://orna.guide/api/v1/${x} -d "{}" | jq . > data/current_entries/guide_api/${x}.json; done
  cp -r data/current_entries data/current_entries-${NOW}
  cd data && BZIP2=-9 tar -cjvf current_entries-${NOW}{.tar.bz2,}
  mv data/current_entries-${NOW}.tar.bz2 data/backups/current_entries/
  rm -r data/current_entries-${NOW}

backup_htmls:
  mv data/htmls data/htmls-${NOW}
  cd data && BZIP2=-9 tar -cjvf htmls-${NOW}{.tar.bz2,}
  mv data/htmls-${NOW}.tar.bz2 data/backups/htmls
  mv data/htmls-${NOW} data/htmls

backup_current_entries:
  cargo run --release --bin ethi json refresh
  cp -r data/current_entries data/current_entries-${NOW}
  cd data && BZIP2=-9 tar -cjvf current_entries-${NOW}{.tar.bz2,}
  mv data/current_entries-${NOW}.tar.bz2 data/backups/current_entries
  rm -r data/current_entries-${NOW}

json_refresh:
  cargo run --release --bin ethi json refresh

backup_htmls_now:
  mv data/htmls data/htmls-${NOWDT}
  cd data && BZIP2=-9 tar -cjvf htmls-${NOWDT}{.tar.bz2,}
  mv data/htmls-${NOWDT}.tar.bz2 data/backups/htmls
  mv data/htmls-${NOWDT} data/htmls

backup_current_entries_now:
  cp -r data/current_entries data/current_entries-${NOWDT}
  cd data && BZIP2=-9 tar -cjvf current_entries-${NOWDT}{.tar.bz2,}
  mv data/current_entries-${NOWDT}.tar.bz2 data/backups/current_entries
  rm -r data/current_entries-${NOWDT}

merge:
  cargo run --release --bin ethi backups merge

quick_merge_now: backup_current_entries_now merge

new_merge_now: json_refresh quick_merge_now

cron: new_jsons backup_htmls backup_current_entries

t:
  echo current_entries-${NOW}
