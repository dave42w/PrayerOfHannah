import datetime
from enum import StrEnum
from typing import List, Optional

from sqlalchemy import TIMESTAMP, String
from sqlalchemy.ext.hybrid import hybrid_property
from sqlalchemy.orm import DeclarativeBase, Mapped, MappedAsDataclass, mapped_column, relationship
from sqlalchemy.schema import ForeignKey, ForeignKeyConstraint, UniqueConstraint


class VerseType(StrEnum):
    VERSE = "v"
    CHORUS = "c"
    BRIDGE = "b"
    ENDING = "e"


class PageType(StrEnum):
    SONG = "s"
    IMAGE = "i"
    VIDEO = "v"
    AUDIO = "a"
    TEXT = "t"


class Base(MappedAsDataclass, DeclarativeBase):
    type_annotation_map = {
        datetime.datetime: TIMESTAMP(timezone=True),
    }


"""
Db structure

----------                 -----------
| Author |                 |Song_Book|
----------                 -----------
    ^                           ^
   1:M        --------         1:M
    |         | Song |          |
    |         --------          |
    |           ^   ^           |
    |          1:M 1:M          |
    |           |   |           |
    ----     ----   ---------   ----
       |     |              |      |
  -------------------   ------------------
  |Author_Song      |   |Song_Book_Item  |
  |M:M no extra data|   |M:M inc Song_Nbr|
  -------------------   | & verse_order  |
                        ------------------
                          ^         ^
                         1:M       1:M
                          |         |
                       -------   -------
                       |Verse|   |Media|
                       -------   -------

A song can have zero, one or many authors, the link is just the relationship

A song can be in zero, one or many Song_Books (held in Song_Book_Item).
Each Song_Book_Item has a number within that Song_Book and a order for the verses

A Song_Book_Item can have zero, one or many Verses. Each Verse has an enumerated type
(V-Verse, C-Chorus, B-Bridge) and a Markdown lyric

A Song_Book_Item can have zero, one or many media files. These have an
enumerated type to control what is displayed
(BI=Background image, V=Video without lyrics, VL=Video with Lyrics, A=Audio only, AS=Audio with Singing, BV=Background video)
They also have a tune name and a verse count

"""


class Author(Base):
    """
    A class to represent the Song/Tune writers.


    Attributes
    ----------
    id : int
        Primary Key, autoincremented
    surname : str
        Surname of the Author
    first_names : str
        All First names and initials of the Author
    songs : list[songs]
        All the songs by this author

    """

    __tablename__: str = "author"

    __table_args__ = (UniqueConstraint("surname", "first_names", name="unique_author_surname_first_names"),)

    id: Mapped[int] = mapped_column(primary_key=True, init=False)
    surname: Mapped[str] = mapped_column(String(50))  # type: ignore[misc]
    first_names: Mapped[str] = mapped_column(String(50))  # type: ignore[misc]

    songs: Mapped[List["Author_Song"]] = relationship(back_populates="author")  # type: ignore[misc]

    @hybrid_property
    def display_name(self):
        return f"{self.surname}, {self.first_names}"


class Song_Book(Base):
    """
    A class to represent a published collection of Songs/Hymns


    Attributes
    ----------
    id : int
        Primary Key, autoincremented
    code :  str
        Short form of Book identifier eg StF
    name : str
        Name of the Song Book
    url : str
        Book website
    songs : list[Song_Book_Item]
        All the songs in this book

    """

    __tablename__: str = "song_book"
    id: Mapped[int] = mapped_column(primary_key=True, init=False)
    code: Mapped[str] = mapped_column(String(10), index=True, unique=True)  # type: ignore[misc]
    name: Mapped[str] = mapped_column(String(50), index=True, unique=True)  # type: ignore[misc]
    url: Mapped[Optional[str]] = mapped_column(String(200), index=True, unique=True)  # type: ignore[misc]

    songs: Mapped[List["Song_Book_Item"]] = relationship(back_populates="song_book")  # type: ignore[misc]


class Song(Base):
    """
    A class to represent a published song


    Attributes
    ----------
    id : int
        Primary Key, autoincremented
    title : str
        Name of the Song
    authors : list[authors]
        All the authors of this book
    song_book_items : list[song_book_item]
        All the songs in this book
    """

    __tablename__: str = "song"
    id: Mapped[int] = mapped_column(primary_key=True, init=False)
    title: Mapped[str] = mapped_column(String(100), index=True, unique=True)  # type: ignore[misc]

    authors: Mapped[List["Author_Song"]] = relationship(back_populates="song")  # type: ignore[misc]
    song_books: Mapped[List["Song_Book_Item"]] = relationship(back_populates="song")  # type: ignore[misc]


class Author_Song(Base):
    """
    A class to represent the many to many link between author and song


    Attributes
    ----------
    author_id : int
        part of the Primary Key, foreign key to author
    song_id : int
        part of the Primary Key, foreign key to song

    """

    __tablename__: str = "author_song"
    author_id: Mapped[int] = mapped_column(ForeignKey("author.id"), primary_key=True, init=False)
    song_id: Mapped[int] = mapped_column(ForeignKey("song.id"), primary_key=True, init=False)

    author: Mapped["Author"] = relationship(back_populates="songs")  # type: ignore[misc]
    song: Mapped["Song"] = relationship(back_populates="authors")  # type: ignore[misc]


class Song_Book_Item(Base):
    """
    A class to represent the many to many link between song_book and song


    Attributes
    ----------
    song_book_id : int
        foreign key to song_book, part of primary key
    song_id : int
        foreign key to song, part of primary key
    nbr : int
        the Song Nbr in this book
    verse_order : str
        the order verses are displayed (eg V1 C1 V2 B1 C1 V3 C1)
    """

    __tablename__: str = "song_book_item"

    song_book_id: Mapped[int] = mapped_column(ForeignKey("song_book.id"), primary_key=True, init=False)
    song_id: Mapped[int] = mapped_column(ForeignKey("song.id"), primary_key=True, init=False)

    nbr: Mapped[int]  # type: ignore[misc]
    verse_order: Mapped[Optional[str]] = mapped_column(String(50))  # type: ignore[misc]

    song_book: Mapped["Song_Book"] = relationship(back_populates="songs")  # type: ignore[misc]
    song: Mapped["Song"] = relationship(back_populates="song_books")  # type: ignore[misc]

    verses: Mapped[List["Verse"]] = relationship(back_populates="song_book_item")  # type: ignore[misc]


class Verse(Base):
    """
    A class to represent a Verse of a song lyrics


    Attributes
    ----------
    id : int
        Primary Key, autoincremented
    song_book_item_id : int
        part of the Primary Key, foreign key to song_book
    type : str
        the verse type (see enum class VerseType)
    number : int
        the verse nbr
    lyrics : str
        markdown lyrics for this verse
    song_book_item : int
        foreign _key to song_book_item
    """

    __tablename__: str = "verse"

    song_book_id: Mapped[int] = mapped_column(primary_key=True, init=False)
    song_id: Mapped[int] = mapped_column(primary_key=True, init=False)
    type: Mapped[str] = mapped_column(String(1), primary_key=True, init=False)
    number: Mapped[int] = mapped_column(primary_key=True, init=False)
    lyrics: Mapped[str] = mapped_column(String(3000))  # type: ignore[misc]

    song_book_item: Mapped["Song_Book_Item"] = relationship(back_populates="verses")  # type: ignore[misc]

    _table_args__ = (ForeignKeyConstraint([song_book_id, song_id], [Song_Book_Item.song_book_id, Song_Book_Item.song_id], name="fk_verse_to_song_book_item"),)


class Presentation(Base):
    """
    A class to represent a Service or a Noticeboard or other form of presentation


    Attributes
    ----------
    id : int
        Primary Key, autoincremented

    """

    __tablename__: str = "presentation"
    id: Mapped[int] = mapped_column(primary_key=True, init=False)
    name: Mapped[str] = mapped_column(String(50), index=True, unique=True)  # type: ignore[misc]
    when: Mapped[datetime.datetime]  # type: ignore[misc]

    pages: Mapped[List["Page"]] = relationship(back_populates="presentation")  # type: ignore[misc]


class Page(Base):
    __tablename__: str = "page"
    id: Mapped[int] = mapped_column(primary_key=True, init=False)  # type: ignore[misc]
    presentation_id: Mapped[int] = mapped_column(ForeignKey("presentation.id"), init=False)
    type: Mapped[str] = mapped_column(String(1))  # type: ignore[misc]
    title: Mapped[str] = mapped_column(String(100))  # type: ignore[misc]

    presentation: Mapped["Presentation"] = relationship(back_populates="pages")  # type: ignore[misc]
    songs: Mapped[List["Page_Song"]] = relationship(back_populates="page")  # type: ignore[misc]
    images: Mapped[List["Page_Image"]] = relationship(back_populates="page")  # type: ignore[misc]
    videos: Mapped[List["Page_Video"]] = relationship(back_populates="page")  # type: ignore[misc]
    audios: Mapped[List["Page_Audio"]] = relationship(back_populates="page")  # type: ignore[misc]
    markdowns: Mapped[List["Page_Markdown"]] = relationship(back_populates="page")  # type: ignore[misc]


class Page_Song(Base):
    __tablename__: str = "page_song"
    page_id: Mapped[int] = mapped_column(ForeignKey("page.id"), primary_key=True, init=False)  # type: ignore[misc]
    song_id: Mapped[int] = mapped_column(ForeignKey("song.id"), init=False)
    author_id: Mapped[int] = mapped_column()

    page: Mapped["Page"] = relationship(back_populates="songs")  # type: ignore[misc]


class Page_Image(Base):
    __tablename__: str = "page_image"
    page_id: Mapped[int] = mapped_column(ForeignKey("page.id"), primary_key=True, init=False)  # type: ignore[misc]
    image_filename: Mapped[str] = mapped_column(String(100))  # type: ignore[misc]

    page: Mapped["Page"] = relationship(back_populates="images")  # type: ignore[misc]


class Page_Video(Base):
    __tablename__: str = "page_video"
    page_id: Mapped[int] = mapped_column(ForeignKey("page.id"), primary_key=True, init=False)  # type: ignore[misc]
    video_filename: Mapped[str] = mapped_column(String(100))  # type: ignore[misc]

    page: Mapped["Page"] = relationship(back_populates="videos")  # type: ignore[misc]


class Page_Audio(Base):
    __tablename__: str = "page_audio"
    page_id: Mapped[int] = mapped_column(ForeignKey("page.id"), primary_key=True, init=False)  # type: ignore[misc]
    audio_filename: Mapped[str] = mapped_column(String(100))  # type: ignore[misc]

    page: Mapped["Page"] = relationship(back_populates="audios")  # type: ignore[misc]


class Page_Markdown(Base):
    __tablename__: str = "page_markdown"
    page_id: Mapped[int] = mapped_column(ForeignKey("page.id"), primary_key=True, init=False)  # type: ignore[misc]
    markdown_text: Mapped[str] = mapped_column(String(3000))  # type: ignore[misc]

    page: Mapped["Page"] = relationship(back_populates="markdowns")  # type: ignore[misc]
