找到桥接ip 如我的vm和host的桥接ip为172.16.155.1

终端用curl -x http://172.16.155.1:7890 https://自定义.com/

如curl -x http://172.16.155.1:7890 https://google.com/

curl -x http://172.16.155.1:7890 https://github.com/

实现vm上访问host的vpn;



浏览器，在浏览器设置里可以这样设置

![image-20210704234535886](/Users/dali/Library/Application Support/typora-user-images/image-20210704234535886.png)





为了方便设置，可以在vm的ubuntu的主目录下，写一个sh脚本，![image-20210704234714425](/Users/dali/Library/Application Support/typora-user-images/image-20210704234714425.png)

至此，每次打开终端，只需source proxy.sh，即可使vm里的ubuntu走host的vpn

