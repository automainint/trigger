# trigger
Make a POST HTTP request to execute shell script.

##  Usage
- Run `trigger <port> <name> <action>`
- POST request on `http://127.0.0.1:<port>/<name>` to will execute `action` shell script.
```sh
trigger 20400 action test.sh
```
