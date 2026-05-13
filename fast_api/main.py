from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI(title="My Advanced API")


class Item(BaseModel):
    name: str
    price: float
    is_offer: bool | None = None


@app.get("/")
def read_root() -> dict:
    return {"Hello": "World"}


@app.get("/items/{item_id}")
def read_item(item_id: int, q: str | None = None) -> dict:
    return {"item_id": item_id, "query": q}


@app.post("/items/")
def create_item(item: Item) -> dict:
    return {"message": f"Successfully created {item.name}!", "data": item}
