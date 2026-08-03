from datetime import date, datetime  # pragma: no cover

FieldInfo = str | int | float | date | datetime | None  # pragma: no cover
FlatFormInfo = list[dict[str, FieldInfo]]  # pragma: no cover
