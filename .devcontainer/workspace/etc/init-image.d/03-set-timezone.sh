rm -f /etc/timezone /etc/localtime
ln -sfn /usr/share/zoneinfo/${TZ} /etc/localtime
echo ${TZ} >/etc/timezone
