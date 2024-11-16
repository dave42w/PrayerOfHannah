from typing import cast
from sqlalchemy import create_engine
from sqlalchemy import select
from sqlalchemy.engine import Engine
from sqlalchemy.orm import Session
from sqlalchemy.engine import ScalarResult
from sqlalchemy.exc import IntegrityError
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
        title: str ="UpdAnd Can It Be"

        song1: Song = Song(title=title, authors=[], song_books=[])
        session.add(song1)

        code: str ="AddStF"
        name: str = "AddSinging the Faith"
        url: str | None = None

        book1: Song_Book = Song_Book(code=code, name=name, url=url, songs=[])
        session.add(book1)

        nbr: int = 25
        verse_order: str ="V1 V2"
        item1: Song_Book_Item = Song_Book_Item(song_id=song1.id, song_book_id=book1.id, nbr=nbr, verse_order=verse_order, song=song1, song_book=book1)
        session.add(item1)
        session.commit()

        songid: int = song1.id
        bookid: int = book1.id
        itemid: int = item1.id

        assert songid, f"SongId is {songid}"
        assert bookid, f"BookId is {bookid}"
        assert itemid, f"ItemId is {itemid}"

        '''
        # check only 1 row in the 3 tables
        s1: ScalarResult[Author_Song] = session.scalars(select(Author_Song))
        l1 = len(s1.all())
        assert l1 == expected_len, f"Author_Song Count is {l1} should be {expected_len}"

        s2: ScalarResult[Author] = session.scalars(select(Author))
        l2 = len(s2.all())
        assert l2 == expected_len, f"Song Count is {l2} should be {expected_len}"

        s3: ScalarResult[Song] = session.scalars(select(Song))
        l3 = len(s3.all())
        assert l3 == expected_len, f"Author Count is {l3} should be {expected_len}"

        # check from Author to Song
        s4: ScalarResult[Author] = session.scalars(select(Author).where(Author.id==aid))
        r4: Author = cast(Author, s4.first())
        assert r4.id == aid, "author id is {r4.id} should be {aid}"
        assert r4.surname == surname, "author Surname is {r4.surname} should be {surname}"
        assert r4.first_names == first_names, "author first_names is {r4.first_names} should be {first_names}"
        assert r4.display_name == display_name, "author display_name is {r4.display_name} should be {display_name}"
        assert r4.songs, "Should not have empty songs"

        author_songs: List[Author_Song] = r4.songs
        l4 = len(author_songs)
        assert l4 == expected_len, f"Count is {l4} should be {expected_len}"
        song: Song = author_songs[0].song
        assert song.title == title, f"Title is {song.title} should be {title}"

        # check from Song to Author
        s5: ScalarResult[Song] = session.scalars(select(Song).where(Song.id==sid))
        r5: Song = cast(Song, s5.first())
        assert r5.id == sid, "Song id is {r5.id} should be {sid}"
        assert r5.title == title, "song title is {r5.title} should be {title}"
        assert r5.authors, "Should not have empty authors"

        author_songs2: List[Author_Song] = r5.authors
        l5 = len(author_songs2)
        assert l5 == expected_len, f"Count is {l5} should be {expected_len}"
        author: Author = author_songs[0].author
        assert author.display_name == display_name, f"Display Name is {author.display_name} should be {display_name}"
        '''