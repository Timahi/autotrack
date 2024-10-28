export default class Profile {
  private constructor(
    public id: number,
    public name: string,
    public createdAt: Date,
    public updatedAt: Date
  ) {}

  static fromJSON({
    id,
    name,
    createdAt,
    updatedAt,
  }: {
    id: number
    name: string
    createdAt: string
    updatedAt: string
  }): Profile {
    return new Profile(id, name, new Date(createdAt), new Date(updatedAt))
  }
}
