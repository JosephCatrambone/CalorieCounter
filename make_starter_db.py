import csv
import json

def main(empty_db_filename:str, food_nutrition_csv_filename:str, output_db_filename:str):
	with open(food_nutrition_csv_filename, 'rt', encoding='utf-8-sig') as fin:
		# All of these are given in units per 100 grams.
		cin = csv.DictReader(fin)
		nutrition_data = [row for row in cin]
	with open(empty_db_filename, 'rt') as fin:
		base_db = json.load(fin)
		base_db['foods'] = [{
			# Find best match for each row.
			"parent_id": 0,
			"id": int(nd['ID']),
			"name": nd['name'],
			"calories": int(float(nd['Calories'])),
			"carbohydrate": float(nd['Carbohydrate (g)']),
			"protein": float(nd['Protein (g)']),
			"fat": float(nd['Fat (g)']),
			"mass": 100,
			"volume": 1,
			"serving": 100/max(1.0, float(nd.get('Serving Weight 2 (g)', "100") or "100")),
		} for nd in nutrition_data]
		# {"last_food_id":1,"last_meal_id":0,
		# "foods":[
		# 	{"id":1,"name":"","manufacturer":"","tags":"","calories":0,"carbohydrate":0,"fat":0,"protein":0,"mass":100,"volume":0.0,"serving":0.0,"user_defined":false,"ingredients":[]}
		# ],"meals":[]}
	with open(output_db_filename, 'wt') as fout:
		json.dump(base_db, fout)

if __name__=="__main__":
	main("empty.fdb", "food_nutrition_per_100g.csv", "base.fdb")