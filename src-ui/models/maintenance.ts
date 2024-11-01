export class Maintenance {
  private constructor(
    public id: number,
    public vehicleId: number,
    public type: string,
    public description: string | null,
    public performedAt: Date,
    public createdAt: Date,
    public updatedAt: Date
  ) {}

  static from({
    id,
    vehicle_id,
    maintenance_type,
    description,
    performed_at,
    created_at,
    updated_at,
  }: {
    id: number
    vehicle_id: number
    maintenance_type: string
    description: string | null
    performed_at: string
    created_at: string
    updated_at: string
  }) {
    return new Maintenance(
      id,
      vehicle_id,
      maintenance_type,
      description,
      new Date(performed_at + 'Z'),
      new Date(created_at + 'Z'),
      new Date(updated_at + 'Z')
    )
  }
}

export class NewMaintenance {
  private constructor(
    public vehicleId: number,
    public type: string,
    public description: string | null,
    public performedAt: Date,
    public createdAt: Date,
    public updatedAt: Date
  ) {}

  static from({
    vehicleId,
    type,
    description,
    performedAt,
    createdAt,
    updatedAt,
  }: {
    vehicleId: number
    type: string
    description?: string
    performedAt: Date
    createdAt: Date
    updatedAt: Date
  }) {
    return new NewMaintenance(
      vehicleId,
      type,
      description || null,
      performedAt,
      createdAt,
      updatedAt
    )
  }

  toJSON() {
    return {
      vehicle_id: this.vehicleId,
      maintenance_type: this.type,
      description: this.description || undefined,
      performed_at: this.performedAt.toISOString().slice(0, -1),
      created_at: this.createdAt.toISOString().slice(0, -1),
      updated_at: this.updatedAt.toISOString().slice(0, -1),
    }
  }
}

export class EditMaintenance {
  private constructor(
    public updatedAt: Date,
    public vehicleId?: number,
    public type?: string,
    public description?: string,
    public performedAt?: Date
  ) {}

  static from({
    vehicleId,
    type,
    description,
    performedAt,
    updatedAt,
  }: {
    vehicleId?: number
    type?: string
    description?: string
    performedAt?: Date
    updatedAt: Date
  }) {
    return new EditMaintenance(updatedAt, vehicleId, type, description, performedAt)
  }

  toJSON() {
    return {
      vehicle_id: this.vehicleId,
      maintenance_type: this.type,
      description: this.description,
      performed_at: this.performedAt?.toISOString().slice(0, -1),
      updated_at: this.updatedAt.toISOString().slice(0, -1),
    }
  }
}
