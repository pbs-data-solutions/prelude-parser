from datetime import date, datetime
from typing import Dict, List, Union

FieldInfo = Union[str, int, float, date, datetime]
FlatFormInfo = List[Dict[str, FieldInfo]]
