from sqlalchemy import create_engine
from sqlalchemy.engine import Engine
from sqlalchemy.orm import Session
import pytest

from dbs.models import Base
from dbs.models import Song
from dbs.models import Song_Book
from dbs.models import Song_Book_Item


@pytest.fixture
def dbe():
    def _dbe() -> Engine:
        e: Engine = create_engine("sqlite:///:memory:", echo=False)
        Base.metadata.create_all(e)
        return e

    return _dbe


def test_add_song_book_item(dbe) -> None:
    e: Engine = dbe()
    with Session(e) as session:
        title: str = "UpdAnd Can It Be"

        song1: Song = Song(title=title, authors=[], song_books=[])
        session.add(song1)

        code: str = "AddStF"
        name: str = "AddSinging the Faith"
        url: str | None = None

        book1: Song_Book = Song_Book(code=code, name=name, url=url, songs=[])
        session.add(book1)

        nbr: int = 25
        verse_order: str = "V1 V2"
        item1: Song_Book_Item = Song_Book_Item(nbr=nbr, verse_order=verse_order, song=song1, song_book=book1, verses=[])
        session.add(item1)
        session.commit()

        songid: int = song1.id
        bookid: int = book1.id

        assert songid, f"SongId is {songid}"
        assert bookid, f"BookId is {bookid}"
