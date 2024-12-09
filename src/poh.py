from robyn import Robyn, Request

app = Robyn(__file__)


@app.get("/")
def h(request: Request):
    return "Hello, world"


app.start(port=8088, host="0.0.0.0")
