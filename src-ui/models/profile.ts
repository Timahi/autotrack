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
    created_at,
    updated_at,
  }: {
    id: number
    name: string
    created_at: string
    updated_at: string
  }): Profile {
    return new Profile(id, name, new Date(created_at), new Date(updated_at))
  }
}
