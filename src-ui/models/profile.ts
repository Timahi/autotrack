export class Profile {
  private constructor(
    public id: number,
    public name: string,
    public createdAt: Date,
    public updatedAt: Date
  ) {}

  static from({
    id,
    name,
    created_at,
    updated_at,
  }: {
    id: number
    name: string
    created_at: string
    updated_at: string
  }) {
    created_at += 'Z'
    updated_at += 'Z'

    return new Profile(id, name, new Date(created_at), new Date(updated_at))
  }
}

export class NewProfile {
  private constructor(
    public name: string,
    public createdAt: Date,
    public updatedAt: Date
  ) {}

  static from({ name }: { name: string }) {
    return new NewProfile(name, new Date(), new Date())
  }

  toJSON() {
    return {
      name: this.name,
      created_at: this.createdAt.toISOString().slice(0, -1),
      updated_at: this.updatedAt.toISOString().slice(0, -1),
    }
  }
}

export class EditProfile {
  private constructor(
    public name: string,
    public updatedAt: Date
  ) {}

  static from({ name }: { name: string }) {
    return new EditProfile(name, new Date())
  }

  toJSON() {
    return { name: this.name, updated_at: this.updatedAt.toISOString().slice(0, -1) }
  }
}
