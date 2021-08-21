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
			"name": nd['Name'],
			"manufacturer": "",
            "nutrition":{
                "calories": int(float(nd['Calories'])),
                "carbohydrates": float(nd['Carbohydrate (g)']),
                "proteins": float(nd['Protein (g)']),
                "fats": float(nd['Fat (g)']),
            },
			"mass": 100,
			"volume_of_100g": 1,
			"servings_in_100g": 100/max(1.0, float(nd.get('Serving Weight 2 (g)', "100") or "100")),
			"user_defined":False,
			"ingredients":[],
			"tags":"",
		} for nd in nutrition_data]
		# {"foods":[{"parent_id":0,"id":0,"name":"Tasty Food","manufacturer":"","tags":"","nutrition":{"calories":260,"fats":20.0,"carbohydrates":60.0,"proteins":20.0},"mass":100,"volume_of_100g":0.0,"servings_in_100g":0.0,"user_defined":true,"ingredients":[]}],"meals":[]}
		base_db['meals'] = []
	with open(output_db_filename, 'wt') as fout:
		json.dump(base_db, fout)

if __name__=="__main__":
	main("empty.fdb", "MyFoodData_Nutrition_Facts_SpreadSheet_Release_1_4.csv", "default.fdb")
    