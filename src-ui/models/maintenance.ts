export class Maintenance {
  private constructor(
    public id: number,
    public vehicleId: number,
    public type: string,
    public description: string | null,
    public odometer: number,
    public performedAt: Date,
    public createdAt: Date,
    public updatedAt: Date
  ) {}

  static from({
    id,
    vehicle_id,
    type,
    description,
    odometer,
    performed_at,
    created_at,
    updated_at,
  }: {
    id: number
    vehicle_id: number
    type: string
    description: string | null
    odometer: number
    performed_at: string
    created_at: string
    updated_at: string
  }) {
    return new Maintenance(
      id,
      vehicle_id,
      type,
      description,
      odometer,
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
    public odometer: number,
    public performedAt: Date,
    public createdAt: Date,
    public updatedAt: Date
  ) {}

  static from({
    vehicleId,
    type,
    description,
    odometer,
    performedAt,
    createdAt,
    updatedAt,
  }: {
    vehicleId: number
    type: string
    description?: string
    odometer: number
    performedAt: Date
    createdAt: Date
    updatedAt: Date
  }) {
    return new NewMaintenance(
      vehicleId,
      type,
      description || null,
      odometer,
      performedAt,
      createdAt,
      updatedAt
    )
  }

  toJSON() {
    return {
      vehicle_id: this.vehicleId,
      type: this.type,
      description: this.description || undefined,
      odometer: this.odometer,
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
    public odometer?: number,
    public performedAt?: Date
  ) {}

  static from({
    vehicleId,
    type,
    description,
    odometer,
    performedAt,
    updatedAt,
  }: {
    vehicleId?: number
    type?: string
    description?: string
    odometer?: number
    performedAt?: Date
    updatedAt: Date
  }) {
    return new EditMaintenance(updatedAt, vehicleId, type, description, odometer, performedAt)
  }

  toJSON() {
    return {
      vehicle_id: this.vehicleId,
      type: this.type,
      description: this.description,
      odometer: this.odometer,
      performed_at: this.performedAt?.toISOString().slice(0, -1),
      updated_at: this.updatedAt.toISOString().slice(0, -1),
    }
  }
}
