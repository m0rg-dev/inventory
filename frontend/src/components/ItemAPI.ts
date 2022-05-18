export default class Item {
  private id: string;
  // TODO this shouldn't be public...
  public tags: { [k: string]: string };

  public constructor(id: string, tags: { [k: string]: string }) {
    this.id = id;
    this.tags = Object.assign({}, tags);
  }

  public static async load(id: string): Promise<Item> {
    return new Item(id, (await (await fetch(`/api/items/${id}`)).json())["tags"]);
  }

  public async save() {
    await fetch(`/api/items/`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(this),
    });
  }

  public async delete() {
    await fetch(`/api/items/${this.id}`, {
      method: "DELETE",
    });
  }

  public async checkOut() {
    this.tags["_checked_out_at"] = new Date().toISOString();
    this.tags["last_checked_out"] = this.tags["_checked_out_at"];
    this.save();
  }

  public async checkIn() {
    delete this.tags["_checked_out_at"];
    this.save();
  }

  public async updateTag(k: string, v: string) {
    this.tags[k] = v;
    this.save();
  }

  public async deleteTag(k: string) {
    delete this.tags[k];
    this.save();
  }

  public checkedOutAt(): string {
    return this.tags["_checked_out_at"];
  }

  public getDescription(): string {
    return this.tags["_description"];
  }

  public async setDescription(description: string) {
    this.tags["_description"] = description;
    this.save();
  }

  public getParent(): string {
    return this.tags["_parent"];
  }

  public async setParent(parent: string) {
    this.tags["_parent"] = parent;
    this.save();
  }

  public async removeParent() {
    this.deleteTag("_parent");
    this.save();
  }

  public getID(): string {
    return this.id;
  }
}
