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

  static fromString(text: string): Profile {
    const parsed = JSON.parse(text)

    if (
      typeof parsed.id !== 'number' ||
      typeof parsed.name !== 'string' ||
      typeof parsed.createdAt !== 'string' ||
      typeof parsed.updatedAt !== 'string'
    ) {
      throw new Error('Invalid data')
    }

    return this.fromJSON(parsed)
  }

  toJson(): {
    id: number
    name: string
    createdAt: string
    updatedAt: string
  } {
    return {
      id: this.id,
      name: this.name,
      createdAt: this.createdAt.toString(),
      updatedAt: this.updatedAt.toString(),
    }
  }

  toString(): string {
    return JSON.stringify(this.toJson())
  }
}
