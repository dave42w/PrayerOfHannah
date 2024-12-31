from robyn import Request, Robyn

app = Robyn(__file__)


@app.get("/")
def h(request: Request):
    return "Hello, world"


def main():
    app.start(port=8080, host="0.0.0.0")


if __name__ == "__main__":
    main()
