from datetime import datetime
from typing import cast
from sqlalchemy import create_engine
from sqlalchemy import select
from sqlalchemy.engine import Engine
from sqlalchemy.orm import Session
from sqlalchemy.engine import ScalarResult
import pytest

from dbs.models import Base
from dbs.models import Presentation

@pytest.fixture
def dbe():
    def _dbe() -> Engine:
        e: Engine = create_engine("sqlite:///:memory:", echo=False)
        Base.metadata.create_all(e)
        return e
    return _dbe


def test_add_presentation(dbe) -> None:
    e: Engine = dbe()
    with Session(e) as session:
        name: str ="Lawton Moor"
        when = datetime.now()
        expected_len: int = 1

        p1: Presentation = Presentation(name=name, when=when)
        session.add(p1)
        session.commit()

        s1: ScalarResult[Presentation] = session.scalars(select(Presentation))
        l1 = len(s1.all())
        assert l1 == expected_len, f"Count is {l1} should be {expected_len}"

        s2: ScalarResult[Presentation] = session.scalars(select(Presentation))
        r2: Presentation = cast(Presentation, s2.first())
        assert r2.name == name, "Add presentation name is {r2.name} should be {name}"
        assert r2.when == when, "Add presentation when is {r2.when} should be {when}"
