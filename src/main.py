from sqlalchemy.engine import Engine
from sqlalchemy import create_engine
from sqlalchemy.orm import Session
from dbs.models import Base
from dbs.models import Author

print("Hello world")


e: Engine = create_engine("sqlite:///:memory:", echo=False)
Base.metadata.create_all(e)

with Session(e) as session:
    surname: str = "AddWarnock"
    first_names: str = "AddDave Z"
    display_name: str = f"{surname}, {first_names}"
    expected_len: int = 1

    a1: Author = Author(surname=surname, first_names=first_names, songs=[])
    session.add(a1)
    session.commit()
